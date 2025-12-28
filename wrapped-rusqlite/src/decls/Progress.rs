macro_rules! Progress {
    () => {
        # [doc = " Struct specifying the progress of a backup."] # [doc = ""] # [doc = " The percentage completion can be calculated as `(pagecount - remaining) /"] # [doc = " pagecount`. The progress of a backup is as of the last call to"] # [doc = " [`step`](Backup::step) - if the source database is modified after a call to"] # [doc = " [`step`](Backup::step), the progress value will become outdated and"] # [doc = " potentially incorrect."] # [derive (Copy , Clone , Debug)] pub struct Progress { # [doc = " Number of pages in the source database that still need to be backed up."] pub remaining : c_int , # [doc = " Total number of pages in the source database."] pub pagecount : c_int , }
    };
}

Progress!()