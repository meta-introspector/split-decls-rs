macro_rules! deps {
    () => {
        Connection!();
    };
}

macro_rules! Backup {
    () => {
        deps!();
        # [doc = " A handle to an online backup."] pub struct Backup < 'a , 'b > { phantom_from : PhantomData < & 'a Connection > , to : & 'b Connection , b : * mut ffi :: sqlite3_backup , }
    };
}

Backup!();