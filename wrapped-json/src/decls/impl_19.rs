macro_rules! deps {
    () => {
        NumberDeserializer!();
        Result!();
        Value!();
        Error!();
        ParserNumber!();
        Float!();
    };
}

macro_rules! impl_19 {
    () => {
        deps!();
        impl ParserNumber { fn visit < 'de , V > (self , visitor : V) -> Result < V :: Value > where V : de :: Visitor < 'de > , { match self { ParserNumber :: F64 (x) => visitor . visit_f64 (x) , ParserNumber :: U64 (x) => visitor . visit_u64 (x) , ParserNumber :: I64 (x) => visitor . visit_i64 (x) , # [cfg (feature = "arbitrary_precision")] ParserNumber :: String (x) => visitor . visit_map (NumberDeserializer { number : x . into () }) , } } fn invalid_type (self , exp : & dyn Expected) -> Error { match self { ParserNumber :: F64 (x) => de :: Error :: invalid_type (Unexpected :: Float (x) , exp) , ParserNumber :: U64 (x) => de :: Error :: invalid_type (Unexpected :: Unsigned (x) , exp) , ParserNumber :: I64 (x) => de :: Error :: invalid_type (Unexpected :: Signed (x) , exp) , # [cfg (feature = "arbitrary_precision")] ParserNumber :: String (_) => de :: Error :: invalid_type (Unexpected :: Other ("number") , exp) , } } }
    };
}

impl_19!()