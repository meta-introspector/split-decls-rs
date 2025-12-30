// Generated macro for not_in (function)
macro_rules! Depcrate_de_mapnot_in {
() => {
// Module: crate::de::map
// Provides: {"not_in"}
// Dependencies: {}
# [doc = " Check if tag `start` is included in the `fields` list. `decoder` is used to"] # [doc = " get a string representation of a tag."] # [doc = ""] # [doc = " Returns `true`, if `start` is not in the `fields` list and `false` otherwise."] fn not_in (fields : & 'static [& 'static str] , start : & BytesStart) -> Result < bool , DeError > { let tag = start . decoder () . decode (start . local_name () . into_inner ()) ? ; Ok (fields . iter () . all (| & field | field != tag . as_ref ())) }
};
}
