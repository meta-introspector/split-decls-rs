macro_rules! mockable_foreign_mod {
    () => {
        # [doc = " Performs transformations on a Foreign Mod to make it mockable"] fn mockable_foreign_mod (mut ifm : ItemForeignMod) -> ItemForeignMod { for item in & mut ifm . items { if let ForeignItem :: Fn (ref mut f) = item { fix_elipses (& mut f . sig) ; } } ifm }
    };
}

mockable_foreign_mod!();