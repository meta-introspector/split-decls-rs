// Generated macro for naively_apply_preferences (function)
macro_rules! Depcrate_provider_pattern_hour_cyclenaively_apply_preferences {
() => {
// Module: crate::provider::pattern::hour_cycle
// Provides: {"naively_apply_preferences"}
// Dependencies: {}
# [doc = " The hour cycle can be set by preferences. This function switches between h11 and h12,"] # [doc = " and between h23 and h24. This function is naive as it is assumed that this application of"] # [doc = " the hour cycle will not change between h1x to h2x."] # [cfg (feature = "datagen")] pub (crate) fn naively_apply_preferences (pattern : & mut runtime :: Pattern , hour_cycle : Option < HourCycle > ,) { if let Some (hour_cycle) = hour_cycle { runtime :: helpers :: maybe_replace_first (pattern , | item | { if let PatternItem :: Field (fields :: Field { symbol : fields :: FieldSymbol :: Hour (current_hour) , length , }) = item { let candidate_field = fields :: Hour :: from_hour_cycle (hour_cycle) ; if * current_hour != candidate_field { Some (PatternItem :: from ((fields :: FieldSymbol :: Hour (candidate_field) , * length ,))) } else { None } } else { None } }) ; } }
};
}
