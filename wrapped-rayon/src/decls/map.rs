macro_rules! map {
    () => {
        # [doc = " ```compile_fail,E0277\n\nuse rayon::prelude::*;\nuse std::ptr::null;\n\n#[derive(Copy, Clone)]\nstruct NoSend(*const ());\n\nunsafe impl Sync for NoSend {}\n\nlet x = Some(NoSend(null()));\n\nx.par_iter()\n    .map(|&x| x) //~ ERROR\n    .count(); //~ ERROR\n\n``` "] mod map { }
    };
}

map!()