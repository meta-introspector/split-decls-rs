macro_rules! LifetimeSyntaxCategories {
    () => {
        # [derive (Debug , Default)] pub struct LifetimeSyntaxCategories < T > { pub hidden : T , pub elided : T , pub named : T , }
    };
}

LifetimeSyntaxCategories!()