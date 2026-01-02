mkuse!{use crate :: detect :: { Feature , cache } ;}

macro_rules! imply_features_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function imply_features in module {}", module_path!());
    };
}

mkfn!{
    imply_features_introspect!();
    # [doc = " Imply features by the given set of enabled features."] # [doc = ""] # [doc = " Note that it does not perform any consistency checks including existence of"] # [doc = " conflicting extensions and/or complicated requirements.  Eliminating such"] # [doc = " inconsistencies is the responsibility of the feature detection logic and"] # [doc = " its provider(s)."] pub (crate) fn imply_features (mut value : cache :: Initializer) -> cache :: Initializer { loop { let prev = value ; macro_rules ! imply { ($ ($ from : ident) |+ => $ ($ to : ident) &+) => { if [$ (Feature ::$ from as u32) ,+] . iter () . any (|& x | value . test (x)) { $ (value . set (Feature ::$ to as u32) ;) + } } ; ($ ($ from : ident) &+ => $ ($ to : ident) &+) => { if [$ (Feature ::$ from as u32) ,+] . iter () . all (|& x | value . test (x)) { $ (value . set (Feature ::$ to as u32) ;) + } } ; } macro_rules ! group { ($ group : ident == $ ($ member : ident) &+) => { imply ! ($ group => $ ($ member) &+) ; imply ! ($ ($ member) &+ => $ group) ; } ; } imply ! (zvbb => zvkb) ; group ! (zvkn == zvkned & zvknhb & zvkb & zvkt) ; group ! (zvknc == zvkn & zvbc) ; group ! (zvkng == zvkn & zvkg) ; group ! (zvks == zvksed & zvksh & zvkb & zvkt) ; group ! (zvksc == zvks & zvbc) ; group ! (zvksg == zvks & zvkg) ; imply ! (zvknhb => zvknha) ; imply ! (zvknhb | zvbc => zve64x) ; imply ! (zvbb | zvkb | zvkg | zvkned | zvknha | zvksed | zvksh => zve32x) ; imply ! (zbc => zbkc) ; group ! (zkn == zbkb & zbkc & zbkx & zkne & zknd & zknh) ; group ! (zks == zbkb & zbkc & zbkx & zksed & zksh) ; group ! (zk == zkn & zkr & zkt) ; imply ! (zabha | zacas => zaamo) ; group ! (a == zalrsc & zaamo) ; group ! (b == zba & zbb & zbs) ; imply ! (zcf => zca & f) ; imply ! (zcd => zca & d) ; imply ! (zcmop | zcb => zca) ; imply ! (zhinx => zhinxmin) ; imply ! (zdinx | zhinxmin => zfinx) ; imply ! (zvfh => zvfhmin) ; imply ! (zvfh => zve32f & zfhmin) ; imply ! (zvfhmin => zve32f) ; imply ! (zvfbfwma => zvfbfmin & zfbfmin) ; imply ! (zvfbfmin => zve32f) ; imply ! (v => zve64d) ; imply ! (zve64d => zve64f & d) ; imply ! (zve64f => zve64x & zve32f) ; imply ! (zve64x => zve32x) ; imply ! (zve32f => zve32x & f) ; imply ! (zfh => zfhmin) ; imply ! (q => d) ; imply ! (d | zfhmin | zfa => f) ; imply ! (zfbfmin => f) ; imply ! (c => zca) ; imply ! (c & d => zcd) ; # [cfg (target_arch = "riscv32")] imply ! (c & f => zcf) ; cfg_select ! { target_arch = "riscv32" => { if value . test (Feature :: d as u32) { imply ! (zcf & zcd => c) ; } else if value . test (Feature :: f as u32) { imply ! (zcf => c) ; } else { imply ! (zca => c) ; } } _ => { if value . test (Feature :: d as u32) { imply ! (zcd => c) ; } else { imply ! (zca => c) ; } } } imply ! (zicntr | zihpm | f | zfinx | zve32x => zicsr) ; if prev == value { return value ; } } }
}
mkmod!{tests, { 
                getname!(tests);
                getsrc!(tests);
                getpath!(tests);
                get_deps!(tests);
                get_crates!(tests);
                mkinclude!(tests);
                 
            }}