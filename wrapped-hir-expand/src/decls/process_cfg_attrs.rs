macro_rules! deps {
    () => {
        MacroCallLoc!();
        ExpandDatabase!();
        ProcMacroKind!();
        ProcMacro!();
        MacroDefKind!();
    };
}

macro_rules! process_cfg_attrs {
    () => {
        deps!();
        pub (crate) fn process_cfg_attrs (db : & dyn ExpandDatabase , node : & SyntaxNode , loc : & MacroCallLoc ,) -> Option < FxHashSet < SyntaxElement > > { let is_derive = match loc . def . kind { MacroDefKind :: BuiltInDerive (..) | MacroDefKind :: ProcMacro (_ , _ , ProcMacroKind :: CustomDerive) => true , MacroDefKind :: BuiltInAttr (_ , expander) => expander . is_derive () , _ => false , } ; let mut remove = FxHashSet :: default () ; let item = ast :: Item :: cast (node . clone ()) ? ; for attr in item . attrs () { if let Some (enabled) = check_cfg_attr (db , & attr , loc . krate) { if enabled { debug ! ("Removing cfg_attr tokens {:?}" , attr) ; let meta = attr . meta () ? ; let removes_from_cfg_attr = remove_tokens_within_cfg_attr (meta) ? ; remove . extend (removes_from_cfg_attr) ; } else { debug ! ("Removing type cfg_attr {:?}" , item . syntax ()) ; remove . insert (attr . syntax () . clone () . into ()) ; } } } if is_derive { match item { ast :: Item :: Struct (it) => match it . field_list () ? { ast :: FieldList :: RecordFieldList (fields) => { process_has_attrs_with_possible_comma (db , fields . fields () , loc . krate , & mut remove ,) ? ; } ast :: FieldList :: TupleFieldList (fields) => { process_has_attrs_with_possible_comma (db , fields . fields () , loc . krate , & mut remove ,) ? ; } } , ast :: Item :: Enum (it) => { process_enum (db , it . variant_list () ? , loc . krate , & mut remove) ? ; } ast :: Item :: Union (it) => { process_has_attrs_with_possible_comma (db , it . record_field_list () ? . fields () , loc . krate , & mut remove ,) ? ; } _ => { } } } Some (remove) }
    };
}

process_cfg_attrs!()