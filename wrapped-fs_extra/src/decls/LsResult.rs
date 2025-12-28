macro_rules! deps {
    () => {
        DirEntryAttr!();
        Result!();
        DirEntryValue!();
    };
}

macro_rules! LsResult {
    () => {
        deps!();
        # [doc = " Result returned by the `ls` function."] pub struct LsResult { # [doc = " Base folder target path"] pub base : HashMap < DirEntryAttr , DirEntryValue > , # [doc = " Collection directory entry with information."] pub items : Vec < HashMap < DirEntryAttr , DirEntryValue > > , }
    };
}

LsResult!()