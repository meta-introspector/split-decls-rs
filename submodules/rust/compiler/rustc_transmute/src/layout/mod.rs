mkuse!{use std :: fmt :: { self , Debug } ;}
mkuse!{use std :: hash :: Hash ;}
mkuse!{use std :: ops :: RangeInclusive ;}
mkmod!{tree, { 
                getname!(tree);
                getsrc!(tree);
                getpath!(tree);
                get_deps!(tree);
                get_crates!(tree);
                mkinclude!(tree);
                 
            }}
mkuse!{pub (crate) use tree :: Tree ;}
mkmod!{dfa, { 
                getname!(dfa);
                getsrc!(dfa);
                getpath!(dfa);
                get_deps!(dfa);
                get_crates!(dfa);
                mkinclude!(dfa);
                 
            }}
mkuse!{pub (crate) use dfa :: { Dfa , union } ;}
mkitem!{mkstruct!{# [derive (Debug)] pub (crate) struct Uninhabited ;}}
mkitem!{mkstruct!{# [doc = " A range of byte values (including an uninit byte value)."] # [derive (Hash , Eq , PartialEq , Ord , PartialOrd , Clone , Copy)] pub (crate) struct Byte { pub (crate) start : u16 , pub (crate) end : u16 , }}}
mkitem!{mkimpl!{impl Byte { const UNINIT : u16 = 256 ; # [inline] fn new (range : RangeInclusive < u8 >) -> Self { let start : u16 = (* range . start ()) . into () ; let end : u16 = (* range . end ()) . into () ; Byte { start , end : end + 1 } } # [inline] fn from_val (val : u8) -> Self { let val : u16 = val . into () ; Byte { start : val , end : val + 1 } } # [inline] fn uninit () -> Byte { Byte { start : 0 , end : Self :: UNINIT + 1 } } # [inline] fn is_empty (& self) -> bool { self . start == self . end } # [inline] fn contains_uninit (& self) -> bool { self . start <= Self :: UNINIT && Self :: UNINIT < self . end } }}}
mkitem!{mkimpl!{impl fmt :: Debug for Byte { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { if self . start == Self :: UNINIT && self . end == Self :: UNINIT + 1 { write ! (f , "uninit") } else if self . start <= Self :: UNINIT && self . end == Self :: UNINIT + 1 { write ! (f , "{}..{}|uninit" , self . start , self . end - 1) } else { write ! (f , "{}..{}" , self . start , self . end) } } }}}
mkitem!{mkimpl!{impl From < RangeInclusive < u8 > > for Byte { fn from (src : RangeInclusive < u8 >) -> Self { Self :: new (src) } }}}
mkitem!{mkimpl!{impl From < u8 > for Byte { # [inline] fn from (src : u8) -> Self { Self :: from_val (src) } }}}
mkitem!{mkstruct!{# [doc = " A reference, i.e., `&'region T` or `&'region mut T`."] # [derive (Debug , Hash , Eq , PartialEq , Ord , PartialOrd , Clone , Copy)] pub (crate) struct Reference < R , T > where R : Region , T : Type , { pub (crate) region : R , pub (crate) is_mut : bool , pub (crate) referent : T , pub (crate) referent_size : usize , pub (crate) referent_align : usize , }}}
mkitem!{mkimpl!{impl < R , T > fmt :: Display for Reference < R , T > where R : Region , T : Type , { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . write_str ("&") ? ; if self . is_mut { f . write_str ("mut ") ? ; } self . referent . fmt (f) } }}}
mkitem!{mktrait!{pub (crate) trait Def : Debug + Hash + Eq + PartialEq + Copy + Clone { fn has_safety_invariants (& self) -> bool ; }}}
mkitem!{mktrait!{pub (crate) trait Region : Debug + Hash + Eq + PartialEq + Copy + Clone { }}}
mkitem!{mktrait!{pub (crate) trait Type : Debug + Hash + Eq + PartialEq + Copy + Clone { }}}
mkitem!{mkimpl!{impl Def for ! { fn has_safety_invariants (& self) -> bool { unreachable ! () } }}}
mkitem!{mkimpl!{impl Region for ! { }}}
mkitem!{mkimpl!{impl Type for ! { }}}
mkitem!{mkimpl!{# [cfg (test)] impl Region for usize { }}}
mkitem!{mkimpl!{# [cfg (test)] impl Type for () { }}}
mkmod!{rustc, { 
                getname!(rustc);
                getsrc!(rustc);
                getpath!(rustc);
                get_deps!(rustc);
                get_crates!(rustc);
                mkinclude!(rustc);
                mkuse!{use rustc_abi :: Layout ;}
mkuse!{use rustc_middle :: ty :: layout :: { HasTyCtxt , LayoutCx , LayoutError } ;}
mkuse!{use rustc_middle :: ty :: { self , Region , Ty } ;}
mkitem!{mkenum!{# [doc = " A visibility node in the layout."] # [derive (Debug , Hash , Eq , PartialEq , Clone , Copy)] pub enum Def < 'tcx > { Adt (ty :: AdtDef < 'tcx >) , Variant (& 'tcx ty :: VariantDef) , Field (& 'tcx ty :: FieldDef) , Primitive , }}}
mkitem!{mkimpl!{impl < 'tcx > super :: Def for Def < 'tcx > { fn has_safety_invariants (& self) -> bool { self != & Self :: Primitive } }}}
mkitem!{mkimpl!{impl < 'tcx > super :: Region for Region < 'tcx > { }}}
mkitem!{mkimpl!{impl < 'tcx > super :: Type for Ty < 'tcx > { }}}

macro_rules! layout_of_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function layout_of in module {}", module_path!());
    };
}

mkfn!{
    layout_of_introspect!();
    pub (crate) fn layout_of < 'tcx > (cx : LayoutCx < 'tcx > , ty : Ty < 'tcx > ,) -> Result < Layout < 'tcx > , & 'tcx LayoutError < 'tcx > > { use rustc_middle :: ty :: layout :: LayoutOf ; let ty = cx . tcx () . erase_and_anonymize_regions (ty) ; cx . layout_of (ty) . map (| tl | tl . layout) }
} 
            }}