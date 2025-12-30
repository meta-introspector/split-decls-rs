// Generated macro for JValueOwned (enum)
macro_rules! Depcrate_jvalueJValueOwned {
() => {
// Module: crate::jvalue
// Provides: {"JValueOwned"}
// Dependencies: {}
# [doc = " A Java owned local reference or primitive value."] # [doc = ""] # [doc = " This type is used for values returned from Java method calls. If the Java"] # [doc = " method returns an object reference, it will take the form of an owned"] # [doc = " [`JObject`]."] # [doc = ""] # [doc = " See also [`JValue`], which is used for Java method call parameters. It is"] # [doc = " different from this type in that it *borrows* an object reference instead"] # [doc = " of owning one."] # [allow (missing_docs)] # [derive (Debug)] pub enum JValueOwned < 'local > { Object (JObject < 'local >) , Byte (jbyte) , Char (jchar) , Short (jshort) , Int (jint) , Long (jlong) , Bool (jboolean) , Float (jfloat) , Double (jdouble) , Void , }
};
}
