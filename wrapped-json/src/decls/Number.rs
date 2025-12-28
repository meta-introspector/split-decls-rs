macro_rules! Number {
    () => {
        # [doc = " Represents a JSON number, whether integer or floating point."] # [derive (Clone , PartialEq , Eq , Hash)] pub struct Number { n : N , }
    };
}

Number!()