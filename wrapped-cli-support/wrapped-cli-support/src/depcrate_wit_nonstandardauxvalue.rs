// Generated macro for AuxValue (enum)
macro_rules! Depcrate_wit_nonstandardAuxValue {
() => {
// Module: crate::wit::nonstandard
// Provides: {"AuxValue"}
// Dependencies: {}
# [doc = " Values that can be imported verbatim to hook up to an import."] # [derive (Debug)] pub enum AuxValue { # [doc = " A bare JS value, no transformations, just put it in the slot."] Bare (JsImport) , # [doc = " A getter function for the class listed for the field, acquired using"] # [doc = " `getOwnPropertyDescriptor`."] Getter (JsImport , String) , # [doc = " Like `Getter`, but accesses a field of a class instead of an instance"] # [doc = " of the class."] ClassGetter (JsImport , String) , # [doc = " Like `Getter`, except the `set` property."] Setter (JsImport , String) , # [doc = " Like `Setter`, but for class fields instead of instance fields."] ClassSetter (JsImport , String) , }
};
}
