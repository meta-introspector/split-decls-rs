// Generated macro for ConfiguredHIR (struct)
macro_rules! Depcrate_configConfiguredHIR {
() => {
// Module: crate::config
// Provides: {"ConfiguredHIR"}
// Dependencies: {}
# [doc = " A \"configured\" HIR expression, which is aware of the configuration which"] # [doc = " produced this HIR."] # [doc = ""] # [doc = " Since the configuration is tracked, values with this type can be"] # [doc = " transformed into other HIR expressions (or regular expressions) in a way"] # [doc = " that preserves the configuration. For example, the `fast_line_regex`"] # [doc = " method will apply literal extraction to the inner HIR and use that to build"] # [doc = " a new regex that matches the extracted literals in a way that is"] # [doc = " consistent with the configuration that produced this HIR. For example, the"] # [doc = " size limits set on the configured HIR will be propagated out to any"] # [doc = " subsequently constructed HIR or regular expression."] # [derive (Clone , Debug)] pub (crate) struct ConfiguredHIR { config : Config , hir : Hir , }
};
}
