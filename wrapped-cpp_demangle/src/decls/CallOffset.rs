macro_rules! deps {
    () => {
        NvOffset!();
        VOffset!();
    };
}

macro_rules! CallOffset {
    () => {
        deps!();
        # [doc = " The `<call-offset>` production."] # [doc = ""] # [doc = " ```text"] # [doc = " <call-offset> ::= h <nv-offset> _"] # [doc = "               ::= v <v-offset> _"] # [doc = " ```"] # [derive (Clone , Debug , PartialEq , Eq)] pub enum CallOffset { # [doc = " A non-virtual offset."] NonVirtual (NvOffset) , # [doc = " A virtual offset."] Virtual (VOffset) , }
    };
}

CallOffset!();