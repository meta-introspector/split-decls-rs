macro_rules! SpawnError {
    () => {
        # [doc = " An error that occurred during spawning."] pub struct SpawnError { _priv : () , }
    };
}

SpawnError!();