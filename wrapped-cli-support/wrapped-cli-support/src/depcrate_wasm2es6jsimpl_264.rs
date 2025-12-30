// Generated macro for impl_264 (impl)
macro_rules! Depcrate_wasm2es6jsimpl_264 {
() => {
// Module: crate::wasm2es6js
// Provides: {"impl_264"}
// Dependencies: {}
impl Output { pub fn typescript (& self) -> Result < String , Error > { let mut ts = typescript (& self . module) ? ; if self . base64 { ts . push_str ("export const booted: Promise<boolean>;\n") ; } Ok (ts) } pub fn js_and_wasm (mut self) -> Result < (String , Option < Vec < u8 > >) , Error > { let mut js_imports = String :: new () ; let mut exports = String :: new () ; let mut set_exports = String :: new () ; let mut imports = String :: new () ; let mut set = HashSet :: new () ; for entry in self . module . imports . iter () { if ! set . insert (& entry . module) { continue ; } let mut name = String :: new () ; push_index_identifier (set . len () , & mut name) ; js_imports . push_str (& format ! ("import * as import_{name} from '{}';\n" , entry . module)) ; imports . push_str (& format ! ("'{}': import_{name}, " , entry . module)) ; } for entry in self . module . exports . iter () { exports . push_str ("export let ") ; exports . push_str (& entry . name) ; exports . push_str (";\n") ; set_exports . push_str (& entry . name) ; set_exports . push_str (" = wasm.exports.") ; set_exports . push_str (& entry . name) ; set_exports . push_str (";\n") ; } if self . unstart () { set_exports . push_str ("wasm.exports.__wasm2es6js_start();\n") ; } let inst = format ! ("
            WebAssembly.instantiate(bytes,{{ {imports} }})
                .then(obj => {{
                    const wasm = obj.instance;
                    {set_exports}
                }})
            " ,) ; let wasm = self . module . emit_wasm () ; let (bytes , booted) = if self . base64 { (format ! ("
                    let bytes;
                    const base64 = \"{base64}\";
                    if (typeof Buffer === 'undefined') {{
                        bytes = Uint8Array.from(atob(base64), c => c.charCodeAt(0));
                    }} else {{
                        bytes = Buffer.from(base64, 'base64');
                    }}
                    " , base64 = BASE64_STANDARD . encode (& wasm)) , inst ,) } else if let Some (ref path) = self . fetch_path { (String :: new () , format ! ("
                    fetch('{path}')
                        .then(res => res.arrayBuffer())
                        .then(bytes => {inst})
                    ") ,) } else { bail ! ("the option --base64 or --fetch is required") ; } ; let js = format ! ("\
            {js_imports}
            {bytes}
            export const booted = {booted};
            {exports}
            " ,) ; let wasm = if self . base64 { None } else { Some (wasm) } ; Ok ((js , wasm)) } # [doc = " See comments above for what this is doing, but in a nutshell this"] # [doc = " removes the start section, if any, and moves it to an exported function."] # [doc = " Returns whether a start function was found and removed."] fn unstart (& mut self) -> bool { let start = match self . module . start . take () { Some (id) => id , None => return false , } ; self . module . exports . add ("__wasm2es6js_start" , start) ; true } }
};
}
