macro_rules! deps {
    () => {
        Map!();
        SerializeMap!();
        MapKeySerializer!();
        Number!();
        RawValue!();
        Error!();
        Result!();
        Value!();
    };
}

macro_rules! impl_352 {
    () => {
        deps!();
        impl serde :: ser :: SerializeMap for SerializeMap { type Ok = Value ; type Error = Error ; fn serialize_key < T > (& mut self , key : & T) -> Result < () > where T : ? Sized + Serialize , { match self { SerializeMap :: Map { next_key , .. } => { * next_key = Some (tri ! (key . serialize (MapKeySerializer))) ; Ok (()) } # [cfg (feature = "arbitrary_precision")] SerializeMap :: Number { .. } => unreachable ! () , # [cfg (feature = "raw_value")] SerializeMap :: RawValue { .. } => unreachable ! () , } } fn serialize_value < T > (& mut self , value : & T) -> Result < () > where T : ? Sized + Serialize , { match self { SerializeMap :: Map { map , next_key } => { let key = next_key . take () ; let key = key . expect ("serialize_value called before serialize_key") ; map . insert (key , tri ! (to_value (value))) ; Ok (()) } # [cfg (feature = "arbitrary_precision")] SerializeMap :: Number { .. } => unreachable ! () , # [cfg (feature = "raw_value")] SerializeMap :: RawValue { .. } => unreachable ! () , } } fn end (self) -> Result < Value > { match self { SerializeMap :: Map { map , .. } => Ok (Value :: Object (map)) , # [cfg (feature = "arbitrary_precision")] SerializeMap :: Number { .. } => unreachable ! () , # [cfg (feature = "raw_value")] SerializeMap :: RawValue { .. } => unreachable ! () , } } }
    };
}

impl_352!();