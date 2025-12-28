macro_rules! VOffset {
    () => {
        # [doc = " A virtual offset, as described by the <v-offset> production."] # [doc = ""] # [doc = " ```text"] # [doc = " <v-offset> ::= <offset number> _ <virtual offset number>"] # [doc = " ```"] # [derive (Clone , Debug , PartialEq , Eq)] pub struct VOffset (isize , isize) ;
    };
}

VOffset!();