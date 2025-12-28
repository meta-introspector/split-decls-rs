macro_rules! deps {
    () => {
        HeadersThenBody!();
        Transport!();
        Error!();
        Http!();
    };
}

macro_rules! impl_99 {
    () => {
        deps!();
        impl < H : Http , B : Unpin > HeadersThenBody < H , B > { fn handle_headers (& mut self) -> std :: io :: Result < () > { if let Some (headers) = self . headers . take () { < Transport < H > > :: check_content_type (self . service , "result" , headers) . map_err (std :: io :: Error :: other) ? ; } Ok (()) } }
    };
}

impl_99!();