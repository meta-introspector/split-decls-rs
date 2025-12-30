// Generated macro for IndexTerm (struct)
macro_rules! Depcrate_solfunmeme_validatorIndexTerm {
() => {
// Module: crate::solfunmeme_validator
// Provides: {"IndexTerm"}
// Dependencies: {}
# [doc = " Structure representing a term in the solfunmeme-index dataset"] # [derive (Debug , Clone , Serialize , Deserialize)] pub struct IndexTerm { pub term : String , pub count : u32 , pub category : String , pub significance : String , pub vibe : String , pub action_suggestion : String , pub emoji_representation : Option < String > , pub semantic_names : Option < Vec < String > > , pub osi_layer : Option < String > , pub prime_factor : Option < u64 > , pub is_power_of_two : Option < bool > , pub numerical_address : Option < String > , pub embedding_vectors : Option < Vec < f64 > > , pub versions : Vec < String > , pub first_seen_timestamp : Option < u64 > , pub last_seen_timestamp : Option < u64 > , }
};
}
