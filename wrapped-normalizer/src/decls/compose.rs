macro_rules! deps {
    () => {
        Composition!();
    };
}

macro_rules! compose {
    () => {
        deps!();
        # [doc = " Performs canonical composition (including Hangul) on a pair of"] # [doc = " characters or returns `None` if these characters don't compose."] # [doc = " Composition exclusions are taken into account."] # [inline] fn compose (iter : Char16TrieIterator , starter : char , second : char) -> Option < char > { let v = u32 :: from (second) . wrapping_sub (HANGUL_V_BASE) ; if v >= HANGUL_JAMO_LIMIT - HANGUL_V_BASE { return compose_non_hangul (iter , starter , second) ; } if v < HANGUL_V_COUNT { let l = u32 :: from (starter) . wrapping_sub (HANGUL_L_BASE) ; if l < HANGUL_L_COUNT { let lv = l * HANGUL_N_COUNT + v * HANGUL_T_COUNT ; return Some (unsafe { char :: from_u32_unchecked (HANGUL_S_BASE + lv) }) ; } return None ; } if in_inclusive_range (second , '\u{11A8}' , '\u{11C2}') { let lv = u32 :: from (starter) . wrapping_sub (HANGUL_S_BASE) ; if lv < HANGUL_S_COUNT && lv % HANGUL_T_COUNT == 0 { let lvt = lv + (u32 :: from (second) - HANGUL_T_BASE) ; return Some (unsafe { char :: from_u32_unchecked (HANGUL_S_BASE + lvt) }) ; } } None }
    };
}

compose!()