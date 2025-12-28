macro_rules! DiagArgValue {
    () => {
        # [doc = " Simplified version of `FluentValue` that can implement `Encodable` and `Decodable`. Converted"] # [doc = " to a `FluentValue` by the emitter to be used in diagnostic translation."] # [derive (Clone , Debug , PartialEq , Eq , Hash , Encodable , Decodable)] pub enum DiagArgValue { Str (Cow < 'static , str >) , Number (i32) , StrListSepByAnd (Vec < Cow < 'static , str > >) , }
    };
}

DiagArgValue!();