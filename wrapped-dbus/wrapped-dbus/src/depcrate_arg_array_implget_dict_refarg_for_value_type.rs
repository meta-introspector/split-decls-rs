// Generated macro for get_dict_refarg_for_value_type (function)
macro_rules! Depcrate_arg_array_implget_dict_refarg_for_value_type {
() => {
// Module: crate::arg::array_impl
// Provides: {"get_dict_refarg_for_value_type"}
// Dependencies: {}
fn get_dict_refarg_for_value_type < 'a , K , KF > (value_type : ArgType , i : & mut Iter < 'a > , kf : KF ,) -> Box < dyn RefArg > where K : DictKey + 'static + RefArg + Clone + Eq + Hash , KF : FnMut (& mut Iter < 'a >) -> Option < K > , { match value_type { ArgType :: Variant => { get_dict_refarg :: < K , Variant < Box < dyn RefArg > > , KF , _ > (i , kf , Variant :: new_refarg) } ArgType :: Byte | ArgType :: Int16 | ArgType :: UInt16 | ArgType :: Int32 | ArgType :: UInt32 | ArgType :: Int64 | ArgType :: UInt64 | ArgType :: Double | ArgType :: String | ArgType :: ObjectPath | ArgType :: Signature | ArgType :: Boolean | ArgType :: UnixFd | ArgType :: Array | ArgType :: Struct => get_internal_dict_refarg :: < K , KF > (i , kf) , ArgType :: DictEntry => panic ! ("Can't have DictEntry as value for dictionary") , ArgType :: Invalid => panic ! ("Array with invalid dictvalue") , } }
};
}
