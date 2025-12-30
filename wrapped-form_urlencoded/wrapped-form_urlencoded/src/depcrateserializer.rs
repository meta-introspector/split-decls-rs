// Generated macro for Serializer (struct)
macro_rules! DepcrateSerializer {
() => {
// Module: crate
// Provides: {"Serializer"}
// Dependencies: {}
# [doc = " The [`application/x-www-form-urlencoded` serializer]("] # [doc = " https://url.spec.whatwg.org/#concept-urlencoded-serializer)."] pub struct Serializer < 'a , T : Target > { target : Option < T > , start_position : usize , encoding : EncodingOverride < 'a > , }
};
}
