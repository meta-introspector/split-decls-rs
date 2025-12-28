macro_rules! Outcome {
    () => {
        # [doc = " The result of [`multi_index::File::write_from_index_paths()`]."] pub struct Outcome { # [doc = " The calculated multi-index checksum of the file at `multi_index_path`."] pub multi_index_checksum : gix_hash :: ObjectId , }
    };
}

Outcome!();