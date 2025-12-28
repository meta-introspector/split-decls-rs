macro_rules! deps {
    () => {
        Ident!();
    };
}

macro_rules! impl_17 {
    () => {
        deps!();
        impl < 's > Ident < 's > { # [doc = " Attempt to decode punycode on the stack (allocation-free),"] # [doc = " and pass the char slice to the closure, if successful."] # [doc = " This supports up to `SMALL_PUNYCODE_LEN` characters."] fn try_small_punycode_decode < F : FnOnce (& [char]) -> R , R > (& self , f : F) -> Option < R > { let mut out = ['\0' ; SMALL_PUNYCODE_LEN] ; let mut out_len = 0 ; let r = self . punycode_decode (| i , c | { out . get (out_len) . ok_or (()) ? ; let mut j = out_len ; out_len += 1 ; while j > i { out [j] = out [j - 1] ; j -= 1 ; } out [i] = c ; Ok (()) }) ; if r . is_ok () { Some (f (& out [.. out_len])) } else { None } } # [doc = " Decode punycode as insertion positions and characters"] # [doc = " and pass them to the closure, which can return `Err(())`"] # [doc = " to stop the decoding process."] fn punycode_decode < F : FnMut (usize , char) -> Result < () , () > > (& self , mut insert : F ,) -> Result < () , () > { let mut punycode_bytes = self . punycode . bytes () . peekable () ; if punycode_bytes . peek () . is_none () { return Err (()) ; } let mut len = 0 ; for c in self . ascii . chars () { insert (len , c) ? ; len += 1 ; } let base = 36 ; let t_min = 1 ; let t_max = 26 ; let skew = 38 ; let mut damp = 700 ; let mut bias = 72 ; let mut i : usize = 0 ; let mut n : usize = 0x80 ; loop { let mut delta : usize = 0 ; let mut w = 1 ; let mut k : usize = 0 ; loop { use core :: cmp :: { max , min } ; k += base ; let t = min (max (k . saturating_sub (bias) , t_min) , t_max) ; let d = match punycode_bytes . next () { Some (d @ b'a' ..= b'z') => d - b'a' , Some (d @ b'0' ..= b'9') => 26 + (d - b'0') , _ => return Err (()) , } ; let d = d as usize ; delta = delta . checked_add (d . checked_mul (w) . ok_or (()) ?) . ok_or (()) ? ; if d < t { break ; } w = w . checked_mul (base - t) . ok_or (()) ? ; } len += 1 ; i = i . checked_add (delta) . ok_or (()) ? ; n = n . checked_add (i / len) . ok_or (()) ? ; i %= len ; let n_u32 = n as u32 ; let c = if n_u32 as usize == n { char :: from_u32 (n_u32) . ok_or (()) ? } else { return Err (()) ; } ; insert (i , c) ? ; i += 1 ; if punycode_bytes . peek () . is_none () { return Ok (()) ; } delta /= damp ; damp = 2 ; delta += delta / len ; let mut k = 0 ; while delta > ((base - t_min) * t_max) / 2 { delta /= base - t_min ; k += base ; } bias = k + ((base - t_min + 1) * delta) / (delta + skew) ; } } }
    };
}

impl_17!()