macro_rules! DerivedField {
    () => {
        # [derive (FromMeta , Default , Clone)] # [darling (default)] # [doc = " Derivied fields arguments: are used to generate derivied fields."] pub struct DerivedField { pub name : Option < Ident > , pub into : Option < String > , pub with : Option < Path > , # [darling (default)] pub owned : Option < bool > , }
    };
}

DerivedField!();