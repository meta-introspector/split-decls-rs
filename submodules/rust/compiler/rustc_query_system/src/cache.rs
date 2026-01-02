mkuse!{use std :: hash :: Hash ;}
mkuse!{use rustc_data_structures :: fx :: FxHashMap ;}
mkuse!{use rustc_data_structures :: sync :: Lock ;}
mkuse!{use crate :: dep_graph :: { DepContext , DepNodeIndex } ;}
mkitem!{mkstruct!{pub struct Cache < Key , Value > { hashmap : Lock < FxHashMap < Key , WithDepNode < Value > > > , }}}
mkitem!{mkimpl!{impl < Key : Clone , Value : Clone > Clone for Cache < Key , Value > { fn clone (& self) -> Self { Self { hashmap : Lock :: new (self . hashmap . borrow () . clone ()) } } }}}
mkitem!{mkimpl!{impl < Key , Value > Default for Cache < Key , Value > { fn default () -> Self { Self { hashmap : Default :: default () } } }}}
mkitem!{mkimpl!{impl < Key , Value > Cache < Key , Value > { # [doc = " Actually frees the underlying memory in contrast to what stdlib containers do on `clear`"] pub fn clear (& self) { * self . hashmap . borrow_mut () = Default :: default () ; } }}}
mkitem!{mkimpl!{impl < Key : Eq + Hash , Value : Clone > Cache < Key , Value > { pub fn get < Tcx : DepContext > (& self , key : & Key , tcx : Tcx) -> Option < Value > { Some (self . hashmap . borrow () . get (key) ? . get (tcx)) } pub fn insert (& self , key : Key , dep_node : DepNodeIndex , value : Value) { self . hashmap . borrow_mut () . insert (key , WithDepNode :: new (dep_node , value)) ; } }}}
mkitem!{mkstruct!{# [derive (Debug , Clone , Eq , PartialEq)] pub struct WithDepNode < T > { dep_node : DepNodeIndex , cached_value : T , }}}
mkitem!{mkimpl!{impl < T : Clone > WithDepNode < T > { pub fn new (dep_node : DepNodeIndex , cached_value : T) -> Self { WithDepNode { dep_node , cached_value } } pub fn get < Tcx : DepContext > (& self , tcx : Tcx) -> T { tcx . dep_graph () . read_index (self . dep_node) ; self . cached_value . clone () } }}}