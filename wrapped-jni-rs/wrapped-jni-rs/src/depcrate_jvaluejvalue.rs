// Generated macro for JValue (enum)
macro_rules! Depcrate_jvalueJValue {
() => {
// Module: crate::jvalue
// Provides: {"JValue"}
// Dependencies: {}
# [doc = " A Java borrowed local reference or primitive value."] # [doc = ""] # [doc = " This type is used for parameters passed to Java method calls. If the Java"] # [doc = " method is to be passed an object reference, it takes the form of a borrowed"] # [doc = " <code>&[JObject]</code>."] # [doc = ""] # [doc = " See also [`JValueOwned`], which is used for Java method return values. It is"] # [doc = " different from this type in that it *owns* an object reference instead"] # [doc = " of borrowing one."] # [allow (missing_docs)] # [derive (Clone , Copy , Debug)] pub enum JValue < 'obj_ref > { Object (& 'obj_ref JObject < 'obj_ref >) , Byte (jbyte) , Char (jchar) , Short (jshort) , Int (jint) , Long (jlong) , Bool (jboolean) , Float (jfloat) , Double (jdouble) , Void , }
};
}
