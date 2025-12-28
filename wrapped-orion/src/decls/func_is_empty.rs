macro_rules! func_is_empty {
    () => {
        # [doc = " Macro to implement an `is_empty()` function which will return `true` if `self.len() == 0`."] macro_rules ! func_is_empty (() => (# [inline] # [doc = " Return `true` if this object does not hold any data, `false` otherwise."] # [doc = ""] # [doc = " __NOTE__: This method should always return `false`, since there shouldn't be a way"] # [doc = " to create an empty instance of this object."] pub fn is_empty (& self) -> bool { self . original_length == 0 })) ;
    };
}

func_is_empty!();