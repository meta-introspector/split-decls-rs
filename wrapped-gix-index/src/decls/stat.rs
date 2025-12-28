macro_rules! deps {
    () => {
        Time!();
        Stat!();
    };
}

macro_rules! stat {
    () => {
        deps!();
        pub (crate) fn stat (data : & [u8]) -> Option < (entry :: Stat , & [u8]) > { let (ctime_secs , data) = read_u32 (data) ? ; let (ctime_nsecs , data) = read_u32 (data) ? ; let (mtime_secs , data) = read_u32 (data) ? ; let (mtime_nsecs , data) = read_u32 (data) ? ; let (dev , data) = read_u32 (data) ? ; let (ino , data) = read_u32 (data) ? ; let (uid , data) = read_u32 (data) ? ; let (gid , data) = read_u32 (data) ? ; let (size , data) = read_u32 (data) ? ; Some ((entry :: Stat { mtime : entry :: stat :: Time { secs : ctime_secs , nsecs : ctime_nsecs , } , ctime : entry :: stat :: Time { secs : mtime_secs , nsecs : mtime_nsecs , } , dev , ino , uid , gid , size , } , data ,)) }
    };
}

stat!();