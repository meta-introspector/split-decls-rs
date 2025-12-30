// Generated macro for deserialize_change_id (function)
macro_rules! Depcrate_core_config_toml_change_iddeserialize_change_id {
() => {
// Module: crate::core::config::toml::change_id
// Provides: {"deserialize_change_id"}
// Dependencies: {}
fn deserialize_change_id < 'de , D : Deserializer < 'de > > (deserializer : D ,) -> Result < Option < ChangeId > , D :: Error > { let value = toml :: Value :: deserialize (deserializer) ? ; Ok (match value { toml :: Value :: String (s) if s == "ignore" => Some (ChangeId :: Ignore) , toml :: Value :: Integer (i) => Some (ChangeId :: Id (i as usize)) , _ => { return Err (serde :: de :: Error :: custom ("expected \"ignore\" or an integer for change-id" ,)) ; } }) }
};
}
