macro_rules! deps {
    () => {
        Response!();
        ExtensionContext!();
        Extension!();
        ValidationResult!();
        ServerError!();
        Result!();
        AnalyzerExtension!();
        NextRequest!();
        NextValidation!();
    };
}

macro_rules! impl_523 {
    () => {
        deps!();
        # [async_trait :: async_trait] impl Extension for AnalyzerExtension { async fn request (& self , ctx : & ExtensionContext < '_ > , next : NextRequest < '_ >) -> Response { let mut resp = next . run (ctx) . await ; let validation_result = self . validation_result . lock () . await . take () ; if let Some (validation_result) = validation_result { resp = resp . extension ("analyzer" , value ! ({ "complexity" : validation_result . complexity , "depth" : validation_result . depth , }) ,) ; } resp } async fn validation (& self , ctx : & ExtensionContext < '_ > , next : NextValidation < '_ > ,) -> Result < ValidationResult , Vec < ServerError > > { let res = next . run (ctx) . await ? ; * self . validation_result . lock () . await = Some (res) ; Ok (res) } }
    };
}

impl_523!();