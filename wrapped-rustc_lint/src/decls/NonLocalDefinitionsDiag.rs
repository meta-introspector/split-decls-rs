macro_rules! deps {
    () => {
        NonLocalDefinitionsCargoUpdateNote!();
    };
}

macro_rules! NonLocalDefinitionsDiag {
    () => {
        deps!();
        pub (crate) enum NonLocalDefinitionsDiag { Impl { depth : u32 , body_kind_descr : & 'static str , body_name : String , cargo_update : Option < NonLocalDefinitionsCargoUpdateNote > , const_anon : Option < Option < Span > > , doctest : bool , macro_to_change : Option < (String , & 'static str) > , } , MacroRules { depth : u32 , body_kind_descr : & 'static str , body_name : String , doctest : bool , cargo_update : Option < NonLocalDefinitionsCargoUpdateNote > , } , }
    };
}

NonLocalDefinitionsDiag!();