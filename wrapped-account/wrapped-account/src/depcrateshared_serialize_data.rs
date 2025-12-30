// Generated macro for shared_serialize_data (function)
macro_rules! Depcrateshared_serialize_data {
() => {
// Module: crate
// Provides: {"shared_serialize_data"}
// Dependencies: {}
# [cfg (feature = "bincode")] fn shared_serialize_data < T : serde :: Serialize , U : WritableAccount > (account : & mut U , state : & T ,) -> Result < () , bincode :: Error > { if bincode :: serialized_size (state) ? > account . data () . len () as u64 { return Err (Box :: new (bincode :: ErrorKind :: SizeLimit)) ; } bincode :: serialize_into (account . data_as_mut_slice () , state) }
};
}
