macro_rules! deps {
    () => {
        Value!();
        Result!();
        Error!();
        SeqDeserializer!();
    };
}

macro_rules! visit_array {
    () => {
        deps!();
        fn visit_array < 'de , V > (array : Vec < Value > , visitor : V) -> Result < V :: Value , Error > where V : Visitor < 'de > , { let len = array . len () ; let mut deserializer = SeqDeserializer :: new (array) ; let seq = tri ! (visitor . visit_seq (& mut deserializer)) ; let remaining = deserializer . iter . len () ; if remaining == 0 { Ok (seq) } else { Err (serde :: de :: Error :: invalid_length (len , & "fewer elements in array" ,)) } }
    };
}

visit_array!()