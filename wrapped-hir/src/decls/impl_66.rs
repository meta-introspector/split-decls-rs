macro_rules! deps {
    () => {
        HasSource!();
        SelfParam!();
        Function!();
    };
}

macro_rules! impl_66 {
    () => {
        deps!();
        impl HasSource for SelfParam { type Ast = ast :: SelfParam ; fn source (self , db : & dyn HirDatabase) -> Option < InFile < Self :: Ast > > { let InFile { file_id , value } = Function :: from (self . func) . source (db) ? ; value . param_list () . and_then (| params | params . self_param ()) . map (| value | InFile { file_id , value }) } }
    };
}

impl_66!();