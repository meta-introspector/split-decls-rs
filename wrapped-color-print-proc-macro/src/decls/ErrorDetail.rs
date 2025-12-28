macro_rules! ErrorDetail {
    () => {
        # [derive (Debug , PartialEq , Clone)] pub struct ErrorDetail < 'a > { pub input : & 'a str , pub message : String , }
    };
}

ErrorDetail!();