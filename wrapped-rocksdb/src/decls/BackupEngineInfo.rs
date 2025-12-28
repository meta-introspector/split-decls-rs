macro_rules! BackupEngineInfo {
    () => {
        # [doc = " Represents information of a backup including timestamp of the backup"] # [doc = " and the size (please note that sum of all backups' sizes is bigger than the actual"] # [doc = " size of the backup directory because some data is shared by multiple backups)."] # [doc = " Backups are identified by their always-increasing IDs."] pub struct BackupEngineInfo { # [doc = " Timestamp of the backup"] pub timestamp : i64 , # [doc = " ID of the backup"] pub backup_id : u32 , # [doc = " Size of the backup"] pub size : u64 , # [doc = " Number of files related to the backup"] pub num_files : u32 , }
    };
}

BackupEngineInfo!();