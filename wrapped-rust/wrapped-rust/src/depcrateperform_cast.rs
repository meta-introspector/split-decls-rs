// Generated macro for perform_cast (function)
macro_rules! Depcrateperform_cast {
() => {
// Module: crate
// Provides: {"perform_cast"}
// Dependencies: {}
fn perform_cast (operand : & str , cast : & Bitcast) -> String { match cast { Bitcast :: None => operand . to_owned () , Bitcast :: I32ToI64 => format ! ("i64::from({})" , operand) , Bitcast :: F32ToI32 => format ! ("({}).to_bits() as i32" , operand) , Bitcast :: F64ToI64 => format ! ("({}).to_bits() as i64" , operand) , Bitcast :: I64ToI32 => format ! ("{} as i32" , operand) , Bitcast :: I32ToF32 => format ! ("f32::from_bits({} as u32)" , operand) , Bitcast :: I64ToF64 => format ! ("f64::from_bits({} as u64)" , operand) , Bitcast :: F32ToI64 => format ! ("i64::from(({}).to_bits())" , operand) , Bitcast :: I64ToF32 => format ! ("f32::from_bits({} as u32)" , operand) , Bitcast :: I64ToP64 => format ! ("::core::mem::MaybeUninit::new({} as u64)" , operand) , Bitcast :: P64ToI64 => format ! ("{}.assume_init() as i64" , operand) , Bitcast :: PToP64 => { format ! ("{{
                        let mut t = ::core::mem::MaybeUninit::<u64>::uninit();
                        t.as_mut_ptr().cast::<*mut u8>().write({});
                        t
                    }}" , operand) } Bitcast :: P64ToP => { format ! ("{}.as_ptr().cast::<*mut u8>().read()" , operand) } Bitcast :: I32ToP | Bitcast :: LToP => { format ! ("{} as *mut u8" , operand) } Bitcast :: PToI32 | Bitcast :: LToI32 => { format ! ("{} as i32" , operand) } Bitcast :: I32ToL | Bitcast :: I64ToL | Bitcast :: PToL => { format ! ("{} as usize" , operand) } Bitcast :: LToI64 => { format ! ("{} as i64" , operand) } Bitcast :: Sequence (sequence) => { let [first , second] = & * * sequence ; perform_cast (& perform_cast (operand , first) , second) } } }
};
}
