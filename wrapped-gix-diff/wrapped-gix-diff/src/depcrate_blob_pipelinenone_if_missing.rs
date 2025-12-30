// Generated macro for none_if_missing (function)
macro_rules! Depcrate_blob_pipelinenone_if_missing {
() => {
// Module: crate::blob::pipeline
// Provides: {"none_if_missing"}
// Dependencies: {}
fn none_if_missing < T > (res : std :: io :: Result < T >) -> std :: io :: Result < Option < T > > { match res { Ok (data) => Ok (Some (data)) , Err (err) if err . kind () == std :: io :: ErrorKind :: NotFound => Ok (None) , Err (err) => Err (err) , } }
};
}
