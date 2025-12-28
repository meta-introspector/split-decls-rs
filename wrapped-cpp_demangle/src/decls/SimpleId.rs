macro_rules! deps {
    () => {
        TemplateArgs!();
        SourceName!();
    };
}

macro_rules! SimpleId {
    () => {
        deps!();
        # [doc = " The `<simple-id>` production."] # [doc = ""] # [doc = " ```text"] # [doc = " <simple-id> ::= <source-name> [ <template-args> ]"] # [doc = " ```"] # [derive (Clone , Debug , PartialEq , Eq)] pub struct SimpleId (SourceName , Option < TemplateArgs >) ;
    };
}

SimpleId!();