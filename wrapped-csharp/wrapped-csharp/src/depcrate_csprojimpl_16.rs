// Generated macro for impl_16 (impl)
macro_rules! Depcrate_csprojimpl_16 {
() => {
// Module: crate::csproj
// Provides: {"impl_16"}
// Dependencies: {}
impl CSProjectMonoBuilder { pub fn generate (& self) -> Result < () > { let name = & self . name ; let world = & self . world_name . replace ("-" , "_") ; let camel = format ! ("{}World" , world . to_upper_camel_case ()) ; let aot = self . aot ; let maybe_aot = match aot { true => format ! ("<WasmBuildNative>{aot}</WasmBuildNative>") , false => String :: new () , } ; let mut csproj = format ! ("<Project Sdk=\"Microsoft.NET.Sdk\">

        <PropertyGroup>
            <TargetFramework>net9.0</TargetFramework>
            <RuntimeIdentifier>wasi-wasm</RuntimeIdentifier>
            <OutputType>Library</OutputType>
            {maybe_aot}
            <RunAOTCompilation>{aot}</RunAOTCompilation>
            <WasmNativeStrip>false</WasmNativeStrip>
            <WasmSingleFileBundle>true</WasmSingleFileBundle>
            <RootNamespace>{name}</RootNamespace>
            <ImplicitUsings>enable</ImplicitUsings>
            <Nullable>enable</Nullable>
            <AllowUnsafeBlocks>true</AllowUnsafeBlocks>
            <!-- treat these are errors so they are caught during code generation tests -->
            <WarningsAsErrors>CS0105</WarningsAsErrors>
        </PropertyGroup>

        <PropertyGroup>
            <PublishTrimmed>true</PublishTrimmed>
            <AssemblyName>{name}</AssemblyName>
        </PropertyGroup>

        <ItemGroup>
          <NativeFileReference Include=\"{camel}_component_type.o\" Condition=\"Exists('{camel}_component_type.o')\"/>
        </ItemGroup>

        ") ; fs :: write (self . dir . join ("nuget.config") , r#"<?xml version="1.0" encoding="utf-8"?>
        <configuration>
            <config>
                <add key="globalPackagesFolder" value=".packages" />
            </config>
            <packageSources>
                <!--To inherit the global NuGet package sources remove the <clear/> line below -->
                <clear />
                <add key="nuget" value="https://api.nuget.org/v3/index.json" />
                <add key="dotnet9" value="https://pkgs.dev.azure.com/dnceng/public/_packaging/dotnet9/nuget/v3/index.json" />
            </packageSources>
        </configuration>"# ,) ? ; if self . clean_targets { let mut wasm_filename = self . dir . join (name) ; wasm_filename . set_extension ("wasm") ; csproj . push_str (& format ! ("<Target Name=\"CleanAndDelete\"  AfterTargets=\"Clean\">
                <!-- Remove obj folder -->
                <RemoveDir Directories=\"$(BaseIntermediateOutputPath)\" />
                <!-- Remove bin folder -->
                <RemoveDir Directories=\"$(BaseOutputPath)\" />
                <RemoveDir Directories=\"{}\" />
                <RemoveDir Directories=\".packages\" />
            </Target>" , wasm_filename . display ())) ; } csproj . push_str (r#"</Project>
            "# ,) ; fs :: write (self . dir . join (format ! ("{camel}.csproj")) , csproj) ? ; Ok (()) } pub fn aot (& mut self) { self . aot = true ; } pub fn clean (& mut self) -> & mut Self { self . clean_targets = true ; self } }
};
}
