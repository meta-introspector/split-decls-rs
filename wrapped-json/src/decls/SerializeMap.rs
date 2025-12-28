macro_rules! deps {
    () => {
        Map!();
        Number!();
        Value!();
        RawValue!();
    };
}

macro_rules! SerializeMap {
    () => {
        deps!();
        pub enum SerializeMap { Map { map : Map < String , Value > , next_key : Option < String > , } , # [cfg (feature = "arbitrary_precision")] Number { out_value : Option < Value > } , # [cfg (feature = "raw_value")] RawValue { out_value : Option < Value > } , }
    };
}

SerializeMap!();