// Generated macro for impl_153 (impl)
macro_rules! Depcrate_definitionsimpl_153 {
() => {
// Module: crate::definitions
// Provides: {"impl_153"}
// Dependencies: {}
impl DefPath { pub fn make < FN > (krate : CrateNum , start_index : DefIndex , mut get_key : FN) -> DefPath where FN : FnMut (DefIndex) -> DefKey , { let mut data = vec ! [] ; let mut index = Some (start_index) ; loop { debug ! ("DefPath::make: krate={:?} index={:?}" , krate , index) ; let p = index . unwrap () ; let key = get_key (p) ; debug ! ("DefPath::make: key={:?}" , key) ; match key . disambiguated_data . data { DefPathData :: CrateRoot => { assert ! (key . parent . is_none ()) ; break ; } _ => { data . push (key . disambiguated_data) ; index = key . parent ; } } } data . reverse () ; DefPath { data , krate } } # [doc = " Returns a string representation of the `DefPath` without"] # [doc = " the crate-prefix. This method is useful if you don't have"] # [doc = " a `TyCtxt` available."] pub fn to_string_no_crate_verbose (& self) -> String { let mut s = String :: with_capacity (self . data . len () * 16) ; for component in & self . data { write ! (s , "::{}" , component . as_sym (true)) . unwrap () ; } s } # [doc = " Returns a filename-friendly string of the `DefPath`, without"] # [doc = " the crate-prefix. This method is useful if you don't have"] # [doc = " a `TyCtxt` available."] pub fn to_filename_friendly_no_crate (& self) -> String { let mut s = String :: with_capacity (self . data . len () * 16) ; let mut opt_delimiter = None ; for component in & self . data { s . extend (opt_delimiter) ; opt_delimiter = Some ('-') ; write ! (s , "{}" , component . as_sym (true)) . unwrap () ; } s } }
};
}
