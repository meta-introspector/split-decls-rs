mkuse!{use std :: collections :: BTreeSet ;}
mkuse!{use rustc_middle :: mir :: visit :: { PlaceContext , Visitor } ;}
mkuse!{use rustc_middle :: mir :: { Body , Local , Location } ;}

macro_rules! find_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function find in module {}", module_path!());
    };
}

mkfn!{
    find_introspect!();
    # [doc = " Find all uses of (including assignments to) a [`Local`]."] # [doc = ""] # [doc = " Uses `BTreeSet` so output is deterministic."] pub (super) fn find (body : & Body < '_ > , local : Local) -> BTreeSet < Location > { let mut visitor = AllLocalUsesVisitor { for_local : local , uses : BTreeSet :: default () } ; visitor . visit_body (body) ; visitor . uses }
}
mkitem!{mkstruct!{struct AllLocalUsesVisitor { for_local : Local , uses : BTreeSet < Location > , }}}
mkitem!{mkimpl!{impl < 'tcx > Visitor < 'tcx > for AllLocalUsesVisitor { fn visit_local (& mut self , local : Local , _context : PlaceContext , location : Location) { if local == self . for_local { self . uses . insert (location) ; } } }}}