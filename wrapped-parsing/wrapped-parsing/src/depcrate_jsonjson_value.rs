// Generated macro for json_value (function)
macro_rules! Depcrate_jsonjson_value {
() => {
// Module: crate::json
// Provides: {"json_value"}
// Dependencies: {}
fn json_value (input : & str) -> IResult < & str , JsonValue > { use JsonValue :: * ; alt ((value (Null , tag ("null")) , map (boolean , Bool) , map (string , Str) , map (double , Num) , map (array , Array) , map (object , Object) ,)) . parse (input) }
};
}
