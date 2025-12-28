macro_rules! deps {
    () => {
        Folder!();
    };
}

macro_rules! UnzipFolder {
    () => {
        deps!();
        # [doc = " `Folder` that unzips into two other `Folder`s"] struct UnzipFolder < 'a , OP , FA , FB > { op : & 'a OP , left : FA , right : FB , }
    };
}

UnzipFolder!()