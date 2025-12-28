macro_rules! deps {
    () => {
        VTabLogCursor!();
    };
}

macro_rules! impl_652 {
    () => {
        deps!();
        impl Drop for VTabLogCursor < '_ > { fn drop (& mut self) { println ! ("VTabLogCursor::drop(tab={}, cursor={})" , self . vtab () . i_inst , self . i_cursor) ; } }
    };
}

impl_652!();