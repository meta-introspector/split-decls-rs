macro_rules! deps {
    () => {
        Registry!();
        RuleError!();
        MetaType!();
        MetaTypeName!();
    };
}

macro_rules! VisitorContext {
    () => {
        deps!();
        # [doc (hidden)] pub struct VisitorContext < 'a > { pub (crate) registry : & 'a registry :: Registry , pub (crate) variables : Option < & 'a Variables > , pub (crate) errors : Vec < RuleError > , type_stack : Vec < Option < & 'a registry :: MetaType > > , input_type : Vec < Option < MetaTypeName < 'a > > > , fragments : & 'a HashMap < Name , Positioned < FragmentDefinition > > , }
    };
}

VisitorContext!();