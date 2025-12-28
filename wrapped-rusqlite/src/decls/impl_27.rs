macro_rules! deps {
    () => {
        StepResult!();
        Backup!();
        Progress!();
        Name!();
        Connection!();
        Result!();
    };
}

macro_rules! impl_27 {
    () => {
        deps!();
        impl Connection { # [doc = " Back up the `name` database to the given"] # [doc = " destination path."] # [doc = ""] # [doc = " If `progress` is not `None`, it will be called periodically"] # [doc = " until the backup completes."] # [doc = ""] # [doc = " For more fine-grained control over the backup process (e.g.,"] # [doc = " to sleep periodically during the backup or to back up to an"] # [doc = " already-open database connection), see the `backup` module."] # [doc = ""] # [doc = " # Failure"] # [doc = ""] # [doc = " Will return `Err` if the destination path cannot be opened"] # [doc = " or if the backup fails."] pub fn backup < N : Name , P : AsRef < Path > > (& self , name : N , dst_path : P , progress : Option < fn (Progress) > ,) -> Result < () > { use self :: StepResult :: { Busy , Done , Locked , More } ; let mut dst = Self :: open (dst_path) ? ; let backup = Backup :: new_with_names (self , name , & mut dst , MAIN_DB) ? ; let mut r = More ; while r == More { r = backup . step (100) ? ; if let Some (f) = progress { f (backup . progress ()) ; } } match r { Done => Ok (()) , Busy => Err (unsafe { error_from_handle (ptr :: null_mut () , ffi :: SQLITE_BUSY) }) , Locked => Err (unsafe { error_from_handle (ptr :: null_mut () , ffi :: SQLITE_LOCKED) }) , More => unreachable ! () , } } # [doc = " Restore the given source path into the"] # [doc = " `name` database. If `progress` is not `None`, it will be"] # [doc = " called periodically until the restore completes."] # [doc = ""] # [doc = " For more fine-grained control over the restore process (e.g.,"] # [doc = " to sleep periodically during the restore or to restore from an"] # [doc = " already-open database connection), see the `backup` module."] # [doc = ""] # [doc = " # Failure"] # [doc = ""] # [doc = " Will return `Err` if the destination path cannot be opened"] # [doc = " or if the restore fails."] pub fn restore < N : Name , P : AsRef < Path > , F : Fn (Progress) > (& mut self , name : N , src_path : P , progress : Option < F > ,) -> Result < () > { use self :: StepResult :: { Busy , Done , Locked , More } ; let src = Self :: open (src_path) ? ; let restore = Backup :: new_with_names (& src , MAIN_DB , self , name) ? ; let mut r = More ; let mut busy_count = 0_i32 ; 'restore_loop : while r == More || r == Busy { r = restore . step (100) ? ; if let Some (ref f) = progress { f (restore . progress ()) ; } if r == Busy { busy_count += 1 ; if busy_count >= 3 { break 'restore_loop ; } thread :: sleep (Duration :: from_millis (100)) ; } } match r { Done => Ok (()) , Busy => Err (unsafe { error_from_handle (ptr :: null_mut () , ffi :: SQLITE_BUSY) }) , Locked => Err (unsafe { error_from_handle (ptr :: null_mut () , ffi :: SQLITE_LOCKED) }) , More => unreachable ! () , } } }
    };
}

impl_27!();