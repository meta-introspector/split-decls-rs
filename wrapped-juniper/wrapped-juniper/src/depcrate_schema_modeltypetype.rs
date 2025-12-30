// Generated macro for TypeType (enum)
macro_rules! Depcrate_schema_modelTypeType {
() => {
// Module: crate::schema::model
// Provides: {"TypeType"}
// Dependencies: {}
# [derive (Clone , Display)] pub enum TypeType < 'a , S : 'a > { # [display ("{}" , _0 . name () . unwrap ())] Concrete (& 'a MetaType < S >) , # [display ("{}!" , ** _0)] NonNull (Box < TypeType < 'a , S > >) , # [display ("[{}]" , ** _0)] List (Box < TypeType < 'a , S > > , Option < usize >) , }
};
}
