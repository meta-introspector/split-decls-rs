macro_rules! deps {
    () => {
        Value!();
        Values!();
        Deserializer!();
    };
}

macro_rules! StreamDeserializer {
    () => {
        deps!();
        # [doc = " Iterator that deserializes a stream into multiple JSON values."] # [doc = ""] # [doc = " A stream deserializer can be created from any JSON deserializer using the"] # [doc = " `Deserializer::into_iter` method."] # [doc = ""] # [doc = " The data can consist of any JSON value. Values need to be a self-delineating value e.g."] # [doc = " arrays, objects, or strings, or be followed by whitespace or a self-delineating value."] # [doc = ""] # [doc = " ```"] # [doc = " use serde_json::{Deserializer, Value};"] # [doc = ""] # [doc = " fn main() {"] # [doc = "     let data = \"{\\\"k\\\": 3}1\\\"cool\\\"\\\"stuff\\\" 3{}  [0, 1, 2]\";"] # [doc = ""] # [doc = "     let stream = Deserializer::from_str(data).into_iter::<Value>();"] # [doc = ""] # [doc = "     for value in stream {"] # [doc = "         println!(\"{}\", value.unwrap());"] # [doc = "     }"] # [doc = " }"] # [doc = " ```"] pub struct StreamDeserializer < 'de , R , T > { de : Deserializer < R > , offset : usize , failed : bool , output : PhantomData < T > , lifetime : PhantomData < & 'de () > , }
    };
}

StreamDeserializer!()