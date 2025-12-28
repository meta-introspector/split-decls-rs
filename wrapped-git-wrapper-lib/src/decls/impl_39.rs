macro_rules! deps {
    () => {
        FileMetadata!();
    };
}

macro_rules! impl_39 {
    () => {
        deps!();
        impl Default for FileMetadata { fn default () -> Self { FileMetadata { modified : UNIX_EPOCH , len : 0 , hash : String :: new () , git_object_hash : None , is_git_tracked : false , } } }
    };
}

impl_39!();