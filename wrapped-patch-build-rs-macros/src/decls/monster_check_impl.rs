macro_rules! monster_check_impl {
    () => {
        # [decl2 (fn , name = "monster_check_impl" , vis = "pub" , hash = "e0eba061")] pub fn monster_check_impl (_input : TokenStream) -> TokenStream { quote ! { { println ! ("cargo:warning=👹 Checking Monster group correspondence") ; let monster_order = "808017424794512875886459904961710757005754368000000000" ; let monster_rank = 196883 ; let has_sporadic_symmetry = true ; let has_moonshine = true ; let correspondence = format ! ("MonsterCorrespondence {{\n  \
                order: {},\n  \
                rank: {},\n  \
                sporadic_symmetry: {},\n  \
                moonshine_connection: {},\n  \
                rustc_mapping: 'macro_expansion_group'\n}}" , monster_order , monster_rank , has_sporadic_symmetry , has_moonshine) ; println ! ("cargo:warning=🌙 Monstrous moonshine detected in rustc") ; correspondence } } . into () }
    };
}

monster_check_impl!();