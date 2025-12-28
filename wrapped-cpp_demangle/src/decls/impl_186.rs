macro_rules! deps {
    () => {
        IndexStr!();
        CvQualifiers!();
        ExceptionSpec!();
        BareFunctionType!();
        Parse!();
        ParseContext!();
        FunctionType!();
        SubstitutionTable!();
        Result!();
    };
}

macro_rules! impl_186 {
    () => {
        deps!();
        impl Parse for FunctionType { fn parse < 'a , 'b > (ctx : & 'a ParseContext , subs : & 'a mut SubstitutionTable , input : IndexStr < 'b > ,) -> Result < (FunctionType , IndexStr < 'b >) > { try_begin_parse ! ("FunctionType" , ctx , input) ; let (cv_qualifiers , tail) = if let Ok ((cv_qualifiers , tail)) = try_recurse ! (CvQualifiers :: parse (ctx , subs , input)) { (cv_qualifiers , tail) } else { (Default :: default () , input) } ; let (exception_spec , tail) = if let Ok ((exception_spec , tail)) = try_recurse ! (ExceptionSpec :: parse (ctx , subs , tail)) { (Some (exception_spec) , tail) } else { (None , tail) } ; let (transaction_safe , tail) = if let Ok (tail) = consume (b"Dx" , tail) { (true , tail) } else { (false , tail) } ; let tail = consume (b"F" , tail) ? ; let (extern_c , tail) = if let Ok (tail) = consume (b"Y" , tail) { (true , tail) } else { (false , tail) } ; let (bare , tail) = BareFunctionType :: parse (ctx , subs , tail) ? ; let (ref_qualifier , tail) = if let Ok ((ref_qualifier , tail)) = try_recurse ! (RefQualifier :: parse (ctx , subs , tail)) { (Some (ref_qualifier) , tail) } else { (None , tail) } ; let tail = consume (b"E" , tail) ? ; let func_ty = FunctionType { cv_qualifiers : cv_qualifiers , exception_spec : exception_spec , transaction_safe : transaction_safe , extern_c : extern_c , bare : bare , ref_qualifier : ref_qualifier , } ; Ok ((func_ty , tail)) } }
    };
}

impl_186!()