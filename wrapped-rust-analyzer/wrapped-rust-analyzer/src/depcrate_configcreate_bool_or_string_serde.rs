// Generated macro for create_bool_or_string_serde (macro)
macro_rules! Depcrate_configcreate_bool_or_string_serde {
() => {
// Module: crate::config
// Provides: {"create_bool_or_string_serde"}
// Dependencies: {}
macro_rules ! create_bool_or_string_serde { ($ ident : ident <$ bool : literal , $ string : literal >) => { mod $ ident { pub (super) fn deserialize <'de , D > (d : D) -> Result < () , D :: Error > where D : serde :: Deserializer <'de >, { struct V ; impl <'de > serde :: de :: Visitor <'de > for V { type Value = () ; fn expecting (& self , formatter : & mut std :: fmt :: Formatter <'_ >,) -> std :: fmt :: Result { formatter . write_str (concat ! (stringify ! ($ bool) , " or \"" , stringify ! ($ string) , "\"")) } fn visit_bool < E > (self , v : bool) -> Result < Self :: Value , E > where E : serde :: de :: Error , { match v { $ bool => Ok (()) , _ => Err (serde :: de :: Error :: invalid_value (serde :: de :: Unexpected :: Bool (v) , & self ,)) , } } fn visit_str < E > (self , v : & str) -> Result < Self :: Value , E > where E : serde :: de :: Error , { match v { $ string => Ok (()) , _ => Err (serde :: de :: Error :: invalid_value (serde :: de :: Unexpected :: Str (v) , & self ,)) , } } fn visit_enum < A > (self , a : A) -> Result < Self :: Value , A :: Error > where A : serde :: de :: EnumAccess <'de >, { use serde :: de :: VariantAccess ; let (variant , va) = a . variant ::<&'de str > () ?; va . unit_variant () ?; match variant { $ string => Ok (()) , _ => Err (serde :: de :: Error :: invalid_value (serde :: de :: Unexpected :: Str (variant) , & self ,)) , } } } d . deserialize_any (V) } pub (super) fn serialize < S > (serializer : S) -> Result < S :: Ok , S :: Error > where S : serde :: Serializer , { serializer . serialize_str ($ string) } } } ; }
};
}
