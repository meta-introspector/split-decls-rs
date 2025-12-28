macro_rules! deps {
    () => {
        File!();
    };
}

macro_rules! Marker {
    () => {
        deps!();
        # [doc = " Locks a resource to allow related resources to be updated using [files][File]."] # [doc = ""] # [doc = " As opposed to the [File] type this one won't keep the tempfile open for writing and thus consumes no"] # [doc = " system resources, nor can it be persisted."] # [must_use = "A Marker that is immediately dropped doesn't lock a resource meaningfully"] # [derive (Debug)] pub struct Marker { inner : gix_tempfile :: Handle < Closed > , created_from_file : bool , lock_path : PathBuf , }
    };
}

Marker!();