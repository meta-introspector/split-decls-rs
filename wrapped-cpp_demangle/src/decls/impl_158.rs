macro_rules! deps {
    () => {
        Type!();
        FunctionType!();
        PointerToMemberType!();
        VectorType!();
        Substitution!();
        Decltype!();
        TemplateParam!();
        BuiltinType!();
        SourceName!();
        Substitutable!();
        ParseContext!();
        IndexStr!();
        CvQualifiers!();
        Parse!();
        TemplateArgs!();
        SubstitutionTable!();
        ClassEnumType!();
        ArrayType!();
        Result!();
    };
}

macro_rules! impl_158 {
    () => {
        deps!();
        impl Parse for TypeHandle { fn parse < 'a , 'b > (ctx : & 'a ParseContext , subs : & 'a mut SubstitutionTable , input : IndexStr < 'b > ,) -> Result < (TypeHandle , IndexStr < 'b >) > { try_begin_parse ! ("TypeHandle" , ctx , input) ; # [doc = " Insert the given type into the substitution table, and return a"] # [doc = " handle referencing the index in the table where it ended up."] fn insert_and_return_handle < 'a , 'b > (ty : Type , subs : & 'a mut SubstitutionTable , tail : IndexStr < 'b > ,) -> Result < (TypeHandle , IndexStr < 'b >) > { let ty = Substitutable :: Type (ty) ; let idx = subs . insert (ty) ; let handle = TypeHandle :: BackReference (idx) ; Ok ((handle , tail)) } if let Ok ((builtin , tail)) = try_recurse ! (BuiltinType :: parse (ctx , subs , input)) { let handle = TypeHandle :: Builtin (builtin) ; return Ok ((handle , tail)) ; } if let Ok (tail) = consume (b"U" , input) { let (name , tail) = SourceName :: parse (ctx , subs , tail) ? ; let (args , tail) = if let Ok ((args , tail)) = try_recurse ! (TemplateArgs :: parse (ctx , subs , tail)) { (Some (args) , tail) } else { (None , tail) } ; let (ty , tail) = TypeHandle :: parse (ctx , subs , tail) ? ; let ty = Type :: VendorExtension (name , args , ty) ; return insert_and_return_handle (ty , subs , tail) ; } if let Ok ((qualifiers , tail)) = try_recurse ! (CvQualifiers :: parse (ctx , subs , input)) { if tail . len () < input . len () { if ! FunctionType :: starts_with (& tail) { let (ty , tail) = TypeHandle :: parse (ctx , subs , tail) ? ; let ty = Type :: Qualified (qualifiers , ty) ; return insert_and_return_handle (ty , subs , tail) ; } } } if let Ok ((ty , tail)) = try_recurse ! (ClassEnumType :: parse (ctx , subs , input)) { let ty = Type :: ClassEnum (ty) ; return insert_and_return_handle (ty , subs , tail) ; } if let Ok ((sub , tail)) = try_recurse ! (Substitution :: parse (ctx , subs , input)) { if tail . peek () != Some (b'I') { match sub { Substitution :: WellKnown (component) => { return Ok ((TypeHandle :: WellKnown (component) , tail)) ; } Substitution :: BackReference (idx) => { return Ok ((TypeHandle :: BackReference (idx) , tail)) ; } } } } if let Ok ((funty , tail)) = try_recurse ! (FunctionType :: parse (ctx , subs , input)) { let ty = Type :: Function (funty) ; return insert_and_return_handle (ty , subs , tail) ; } if let Ok ((ty , tail)) = try_recurse ! (ArrayType :: parse (ctx , subs , input)) { let ty = Type :: Array (ty) ; return insert_and_return_handle (ty , subs , tail) ; } if let Ok ((ty , tail)) = try_recurse ! (VectorType :: parse (ctx , subs , input)) { let ty = Type :: Vector (ty) ; return insert_and_return_handle (ty , subs , tail) ; } if let Ok ((ty , tail)) = try_recurse ! (PointerToMemberType :: parse (ctx , subs , input)) { let ty = Type :: PointerToMember (ty) ; return insert_and_return_handle (ty , subs , tail) ; } if let Ok ((param , tail)) = try_recurse ! (TemplateParam :: parse (ctx , subs , input)) { if tail . peek () != Some (b'I') { let ty = Type :: TemplateParam (param) ; return insert_and_return_handle (ty , subs , tail) ; } else if ctx . in_conversion () { let mut tmp_subs = subs . clone () ; match try_recurse ! (TemplateArgs :: parse (ctx , & mut tmp_subs , tail)) { Ok ((_ , new_tail)) if new_tail . peek () == Some (b'I') => { } _ => { let ty = Type :: TemplateParam (param) ; return insert_and_return_handle (ty , subs , tail) ; } } } } if let Ok ((ttp , tail)) = try_recurse ! (TemplateTemplateParamHandle :: parse (ctx , subs , input)) { let (args , tail) = TemplateArgs :: parse (ctx , subs , tail) ? ; let ty = Type :: TemplateTemplate (ttp , args) ; return insert_and_return_handle (ty , subs , tail) ; } if let Ok ((param , tail)) = try_recurse ! (Decltype :: parse (ctx , subs , input)) { let ty = Type :: Decltype (param) ; return insert_and_return_handle (ty , subs , tail) ; } if let Ok (tail) = consume (b"P" , input) { let (ty , tail) = TypeHandle :: parse (ctx , subs , tail) ? ; let ty = Type :: PointerTo (ty) ; return insert_and_return_handle (ty , subs , tail) ; } if let Ok (tail) = consume (b"R" , input) { let (ty , tail) = TypeHandle :: parse (ctx , subs , tail) ? ; let ty = Type :: LvalueRef (ty) ; return insert_and_return_handle (ty , subs , tail) ; } if let Ok (tail) = consume (b"O" , input) { let (ty , tail) = TypeHandle :: parse (ctx , subs , tail) ? ; let ty = Type :: RvalueRef (ty) ; return insert_and_return_handle (ty , subs , tail) ; } if let Ok (tail) = consume (b"C" , input) { let (ty , tail) = TypeHandle :: parse (ctx , subs , tail) ? ; let ty = Type :: Complex (ty) ; return insert_and_return_handle (ty , subs , tail) ; } if let Ok (tail) = consume (b"G" , input) { let (ty , tail) = TypeHandle :: parse (ctx , subs , tail) ? ; let ty = Type :: Imaginary (ty) ; return insert_and_return_handle (ty , subs , tail) ; } let tail = consume (b"Dp" , input) ? ; let (ty , tail) = TypeHandle :: parse (ctx , subs , tail) ? ; let ty = Type :: PackExpansion (ty) ; insert_and_return_handle (ty , subs , tail) } }
    };
}

impl_158!();