// Generated macro for parse_phases_string (function)
macro_rules! Depcrateparse_phases_string {
() => {
// Module: crate
// Provides: {"parse_phases_string"}
// Dependencies: {}
# [doc = " Parse phases string into ProcessingPhase enum values"] fn parse_phases_string (phases_str : & str) -> Result < Vec < ProcessingPhase > , ValidationError > { let mut phases = Vec :: new () ; for phase_str in phases_str . split (',') { let phase_str = phase_str . trim () ; let phase = match phase_str { "parsing" => ProcessingPhase :: Parsing , "name_resolution" => ProcessingPhase :: NameResolution , "type_inference" => ProcessingPhase :: TypeInference , "hir_generation" => ProcessingPhase :: HirGeneration , "diagnostics" => ProcessingPhase :: Diagnostics , "completions" => ProcessingPhase :: Completions , "hover" => ProcessingPhase :: Hover , "goto_definition" => ProcessingPhase :: GotoDefinition , "find_references" => ProcessingPhase :: FindReferences , _ => return Err (ValidationError :: InvalidInput (format ! ("Unknown phase: {}" , phase_str))) , } ; phases . push (phase) ; } if phases . is_empty () { return Err (ValidationError :: InvalidInput ("No valid phases specified" . to_string ())) ; } Ok (phases) }
};
}
